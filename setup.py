from pathlib import Path
from setuptools import setup  # type: ignore
 
root = Path(__file__).parent

requirements = (root / "requirements.txt").read_text("utf-8").strip().splitlines()

setup(
    name="nekit.dev",
    author="nekitdev",
    author_email="nekitdevofficial@gmail.com",
    url="https://github.com/nekitdev/nekit.dev",
    project_urls={"Issue tracker": "https://github.com/nekitdev/nekit.dev/issues"},
    version="1.0.0a0",
    packages=["nekitdev", "nekitdev.routes"],
    license="MIT",
    description="nekit.dev - Web Application written in Python with Bulma.",
    include_package_data=True,
    install_requires=requirements,
    python_requires=">=3.6",
    classifiers=[
        "Development Status :: 3 - Alpha",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Natural Language :: English",
        "Operating System :: OS Independent",
    ],
    entry_points={"console_scripts": ["nekit.dev = nekitdev.__main__:main",]},
)
