{:ok, file} = File.read("part1.input")

map = MapSet.new(
  file
  |> String.split
  |> Enum.map(&String.to_integer/1)
)

Enum.each map, fn a ->
  Enum.each map, fn b ->
    Enum.each map, fn c ->
      if a + b + c == 2020 do
        IO.puts a * b * c
      end
    end
  end
end
