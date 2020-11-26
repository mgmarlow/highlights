module Highlights
  class Renderer
    include Formatter::HTML
    include Formatter::Markdown

    def initialize(document, outfile)
      @document = document
      @outfile = outfile
    end

    def self.call(*args)
      new(*args).call
    end

    def call
      case File.extname(@outfile)
      when '.md', '.markdown'
        render_markdown
      when '.html'
        render_html
      end
    end
  end
end
