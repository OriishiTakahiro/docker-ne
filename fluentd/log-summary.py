#!/usr/bin/env python3

import re
import os
import sys
import json
import datetime
import argparse
import csv
import tzlocal

# check parameters
parser = argparse.ArgumentParser(description="Show log summary.")
parser.add_argument("-f", "--format", type=str, default="normal", help="Display format. (normal, csv)")
parser.add_argument("-d", "--dir", type=str, default="/tmp/log/fluentd", help="Directory path of log files.")
args = parser.parse_args()

LOG_PATH_REGEX = r"(node[0-9]+)\.[0-9]+_[0-9]+.log"

# get datetime from log message
def msg2datetime(msg):
    return datetime.datetime( int(msg[0:4]), int(msg[5:7]), int(msg[8:10]), int(msg[11:13]), int(msg[14:16]), int(msg[17:20]), tzinfo=datetime.timezone.utc )

# get last time of records
def last_time(records):
    current = None
    last    = None
    for r in records:
        current = msg2datetime(r["log"])
        if last is None:
            last = current
            continue
        if last < current:
            last = current
    return last

def get_min(records, key):
    min = None
    for v in records.values():
        if min is None:
            min = v[key]
            continue
        if v[key] < min:
            min = v[key]
    return min

def get_max(records, key):
    max = None
    for v in records.values():
        if max is None:
            max = v[key]
            continue
        if v[key] > max:
            max = v[key]
    return max

nodes = {}
log_files = os.listdir(path=args.dir)

# read all files and aggregate records
for file in log_files:
    matched = re.match(LOG_PATH_REGEX, file)
    with open( "{dir}/{file}".format(dir=args.dir, file=file) ) as f:
        node_name = matched.groups()[0]
        if not node_name in nodes:
            nodes[node_name] = []
        for line in f.readlines():
            record = json.loads(line)
            nodes[node_name].append(record)
            line = f.readline

nodes_stats = {}
# read all files and aggregate records
for (name, records) in nodes.items():

    # get start time
    start_records = list(filter(lambda e: "starts" in e["log"], records))
    started_at = msg2datetime( start_records[0]["log"] )

    # get records by regexp
    erouter_records = list(filter(lambda e: "OSPF6: lsdb_hook_add: E-Router LSA" in e["log"], records))
    intra_prefix_records = list(filter(lambda e: "OSPF6: lsdb_hook_add: Intra-Area-Prefix LSA" in e["log"], records))

    erouter_last_time = last_time(erouter_records)
    intra_last_time = last_time(intra_prefix_records)

    nodes_stats[name] = {
        ### E-Router LSA ###
        "erouter_num"               : len(erouter_records),
        # if non E-Router LSA are found, erouter_last_time is None
        "erouter_time_sec"          : (erouter_last_time - started_at).total_seconds() if erouter_last_time is not None else None,
        "erouter_updated_at"        : erouter_last_time if erouter_last_time is not None else datetime.datetime(2020, 1, 1, tzinfo=datetime.timezone.utc),
        ### Intra-Area-Prefix LSA ###
        "intra_prefix_num"          : len(intra_prefix_records),
        # if non Intra-Area-Prefix LSA are found, intra_last_time is None
        "intra_prefix_time_sec"     : (intra_last_time - started_at).total_seconds() if intra_last_time is not None else None,
        "intra_prefix_updated_at"   : intra_last_time if intra_last_time is not None else datetime.datetime(2020, 1, 1, tzinfo=datetime.timezone.utc),
    }

# display as CSV format
if args.format == "csv":
    labels = [
        "Node Name",
        "Number of E-Router LSA",
        "Elapsed Time until Last E-Router LSA (sec)",
        "Number of Intra-Area-Prefix LSA",
        "Elapsed Time until Last Intra-Area-Prefix LSA (sec)",
    ]
    print(", ".join(labels))

    field_names = [ "erouter_num", "erouter_time_sec", "intra_prefix_num", "intra_prefix_time_sec" ]
    writer = csv.DictWriter(sys.stdout, fieldnames=field_names)
    for row in nodes_stats.values():
        writer.writerow(row)
    sys.exit(0)

# display as normal format
if args.format == "normal":
    for k, v in sorted(nodes_stats.items(), key=lambda x: int(x[0][4:])):
        msg = """
About {name}
E-Router LSA:
    Number of LSA               : {erouter_num}
    Elapsed time until last LSA : {erouter_time} (sec)
Intra-Area-Prefix LSA:
    Number of LSA               : {prefix_num}
    Elapsed time until last LSA : {prefix_time} (sec)
-----------------------------------------------------
        """.format(
            name=k,
            erouter_num=v["erouter_num"],
            erouter_time=v["erouter_time_sec"],
            prefix_num=v["intra_prefix_num"],
            prefix_time=v["intra_prefix_time_sec"],
        )
        print(msg)
    print("""
Minimum (Number of LSA):
    E-Router LSA            : {min_erouter}
    Intra-Area-Prefix LSA   : {min_intra}
Maximam (Number of LSA):
    E-Router LSA            : {max_erouter}
    Intra-Area-Prefix LSA   : {max_intra}
Last Updated Time (sec):
    E-Router LSA            : {last_erouter}
    Intra-Area-Prefix LSA   : {last_intra}
    """.format(
        min_erouter = get_min(nodes_stats, "erouter_num"),
        min_intra   = get_min(nodes_stats, "intra_prefix_num"),
        max_erouter = get_max(nodes_stats, "erouter_num"),
        max_intra   = get_max(nodes_stats, "intra_prefix_num"),
        last_erouter= get_max(nodes_stats, "erouter_updated_at").astimezone(tzlocal.get_localzone()),
        last_intra  = get_max(nodes_stats, "intra_prefix_updated_at").astimezone(tzlocal.get_localzone())
    ))
sys.exit(0)
