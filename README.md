# Docker Network Emulator

I checked the operation of this system on following versions.

### Sofware

- OS: Manjaro 18.0.4 Illyria
- Kernel: x86\_64 Linux 4.19.45-1-MANJARO
- Docker:
  - Client: 18.09.6-ce
  - Server: 18.09.6-ce
- Cargo: 1.34.0

This repository include some customized tools, which based these versions.

- Open V Switch: 2.11.0-1
- Quagga: 0.99.19 base

### Preparetion

#### Build and Install OVS(Open V Switch)

I prepared customized ovs which is able to use IPv6 networking.

```sh
$ git clone git@github.com:OriishiTakahiro/ovs.git
$ cd ./ovs
$ git switch 2.11-ovsdocker-ipv6
$ ./boot.sh
$ ./configure
$ make
$ sudo make install
# Start ovs-ctl daemon
$ sudo /usr/local/share/openvswitch/scripts/ovs-ctl start
# Confirm to installed ovs-docker
$ sudo ovs-docker --help
```

> You must start ovs-ctl at every restart.
> If you hassle this process, you make a system unit file and enable its service.

#### Enable to Use IPv6 for Docker

Create `/etc/docker/daemon.json` and write following content in it.

```json
{
  "ipv6": true,
  "fixed-cidr-v6": "2001:db8:1::/64"
}
```
And restart docker daemon.

```sh
$ sudo systemctl restart docker.service
```

### Compile Network-Runner

```sh
# Move root to network-runner directory.
$ cd network-runner
# Build CLI tool.
$ cargo build
# Moves binary file to under PATH.
$ sudo mv ./target/debug/network-runner /usr/local/bin
```

### Running OSPFv6 on Virtual Network

```sh
# Move root to network-runner directory.
$ cd network-runner

# Activate a virtual network.
$ sudo network-runner run ../networks/<file name>.yml

# show help
$ network-runner help

# Deactivate a virtual network.
$ sudo network-runner stop
```

### Manipulate the virtual network

```sh
# show help of network-runner
$ network-runner --help

# Show all nodes
$ network-runner nodes ls

# Show all switches
$ network-runner switches ls

# Create a new switch s0
$ network-runner switches add s0
# Link s0 to node0
$ network-runner switches link s0 --interface eth1 --ip6addr 2001:1200::3 --node node0
# Unlink s0 to node0
$ network-runner switches unlink s0 --interface eth1 --node node0

```
