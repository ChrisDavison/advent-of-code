defmodule AocDay01 do
  def run() do
    {:ok, data} = File.read("input/01")

    nums =
      data
      |> String.split("\n", trim: true)
      |> Enum.map(&String.to_integer/1)

    nums
    |> Enum.map(&required_mass_of_fuel/1)
    |> Enum.sum()
    |> (&"2019 1.1 -> #{&1}").()
    |> IO.puts()

    nums
    |> Enum.map(&required_mass_of_fuel_recursive/1)
    |> Enum.sum()
    |> (&"2019 1.2 -> #{&1}").()
    |> IO.puts()
  end

  def required_mass_of_fuel(fuel) do
    floor(fuel / 3) - 2
  end

  def required_mass_of_fuel_recursive(fuel) do
    result = floor(fuel / 3) - 2

    cond do
      result <= 0 -> 0
      result > 0 -> result + required_mass_of_fuel_recursive(result)
    end
  end
end

defmodule AocDay02 do
  def run() do
    nums =
      File.read!("input/02")
      |> String.split(",")
      |> Enum.map(&String.trim/1)
      |> Enum.map(&String.to_integer/1)
  end
end

# AocDay01.run()
AocDay02.run()
