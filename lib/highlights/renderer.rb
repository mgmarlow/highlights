module Highlights
  class Renderer
    include Formatter::HTML
    include Formatter::Markdown

    def initialize(document, outfile)
      @document = document
      @outfile = outfile
    end

    def self.render(*args)
      new(*args).render
    end

    def render
      case File.extname(@outfile)
      when '.md', '.markdown'
        render_markdown
      when '.html'
        render_html
      end
    end
  end
end
