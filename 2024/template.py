import utility as u
from pathlib import Path
import click


@click.group()
@click.option("--debug/--no-debug", default=False)
def cli(debug):
    print(debug)


@cli.command()
def sample():
    run(Path("input/DAYs").read_text().strip())


@cli.command()
def full():
    run(Path("input/DAY").read_text().strip())


@cli.command()
def stdin():
    run(input("data: "))


def run(data):
    pass


if __name__ == "__main__":
    cli()
