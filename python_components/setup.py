from setuptools import setup, find_packages

setup(
    name="arbitrage_bot",
    version="0.1",
    packages=find_packages(),
    install_requires=[
        "requests>=2.26.0",
        "setuptools>=58.0.0",
    ],
)
