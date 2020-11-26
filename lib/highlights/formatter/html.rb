module Highlights
  module Formatter
    module HTML
      def render_html
        file = File.join(File.dirname(__FILE__), "./template.html.erb")
        template = File.read(file)

        File.open(@outfile, 'w') do |file|
          file.write(ERB.new(template).result(binding))
        end
      end
    end
  end
end

