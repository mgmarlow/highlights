module Highlights
  class Parser
    class MalformedCSVError < Error; end
    NOTES_STARTING_POSITION = 8

    def initialize(filename)
      @filename = filename
    end

    def parse
      table = CSV.read(@filename, liberal_parsing: true)

      notes = table[NOTES_STARTING_POSITION...].map do |note_row|
        Note.new(*note_row)
      end

      Document.new(table[1][0], table[2][0], notes)
    rescue StandardError => e
      raise MalformedCSVError.new(e.message)
    end
  end
end
