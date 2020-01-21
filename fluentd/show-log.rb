#!/usr/bin/env ruby
require 'json'
puts readlines.map{|r| JSON.parse( r.match(/({.+})/)[1] )['log'] rescue r }
