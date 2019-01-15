import sys

from setuptools import setup
from setuptools.command.test import test as TestCommand
from setuptools_rust import RustExtension, Binding

setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = []

setup(
    name="similarity-hash",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    packages=["fuzzy"],
    rust_extensions=[
        # RustExtension("fuzzy.ssdeep", "Cargo.toml", binding=Binding.RustCPython)
        RustExtension("fuzzy.ssdeep", "Cargo.toml")
    ],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
)