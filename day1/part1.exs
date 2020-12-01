{:ok, file} = File.read("part1.input")

map = MapSet.new(
  file
  |> String.split
  |> Enum.map(&String.to_integer/1)
)

Enum.each map, fn v ->
  x = 2020 - v
  if MapSet.member?(map, x) do
    IO.puts v * x
  end
end
