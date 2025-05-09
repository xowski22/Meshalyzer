from setuptools import setup, find_packages
from setuptools_rust import RustExtension

setup(
    name="meshalyzer",
    version="0.1.0",
    packages=find_packages(),
    rust_extensions=[RustExtension("meshalyzer.meshalyzer", "../Cargo.toml", debug=False)],
    install_requires=[
        "numpy",
        "open3d",
        "matplotlib"
    ],
    zip_safe=False,
)