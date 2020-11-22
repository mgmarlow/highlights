require "csv"
require "optparse"

require "highlights/error"
require "highlights/version"
require "highlights/parser"
require "highlights/renderer"

module Highlights
  Note = Struct.new(:type, :location, :starred, :annotation)
  Document = Struct.new(:title, :author, :notes)
end
