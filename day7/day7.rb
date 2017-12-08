#!/usr/bin/env ruby

class Node
  attr_reader :name, :weight, :children
  attr_accessor :parent

  def inspect
    "<Node #{name} weight=#{weight} children=#{children.length}>"
  end

  def initialize(name, weight, children = [])
    @name     = name
    @weight   = weight
    @children = children
    @parent   = nil
  end

  def self.from_line(line)
    this_node, child_nodes = line.chomp.split(' -> ', 2)

    children = []
    capture = this_node.match(/(?<name>\w+)\s\((?<weight>\d+)\)/)

    children = child_nodes.split(', ') if child_nodes

    new(capture[:name], capture[:weight].to_i, children)
  end

  def total_weight
    @total_weight ||= children.sum(&:total_weight) + weight
  end
end

@nodes = {}

File.readlines('./input.txt').each do |line|
  node = Node.from_line(line)
  @nodes[node.name] = node
end

@nodes.each do |_, node|
  node.children.map! { |child_name| @nodes[child_name] or raise "AMBER ALERT #{child_name}" }
  node.children.each { |child| child.parent = node }
end

root = @nodes.values.first
root = root.parent while root.parent

n = root
loop do
  children = n.children.group_by(&:total_weight).sort_by{ |_, nodes| nodes.length }
  child = children.first.last.first
  expected_weight = children.last.last.first.total_weight

  # if this is true, then current node is the problem
  break if expected_weight == child.total_weight
  diff = child.total_weight - expected_weight
  puts "#{n.name} has child #{child.name}(#{child.weight}) with incorrect weight #{child.total_weight} should be #{expected_weight} (#{diff})"
  n = child
end

#end
