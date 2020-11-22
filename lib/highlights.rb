require "csv"
require "optparse"

require "highlights/error"
require "highlights/version"
require "highlights/cli"
require "highlights/parser"
require "highlights/renderer"

module Highlights
  Note = Struct.new(:type, :location, :starred, :annotation)
  Document = Struct.new(:title, :author, :notes)

  def self.run(args)
    options = CLI.new(args).options
    document = Parser.new(options.filename).parse
    Renderer.new(document, options.output).render
  end
end
