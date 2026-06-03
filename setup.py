from setuptools import setup, find_packages

setup(
    name='webp-converter',
    version='0.1.0',
    description='A simple CLI tool to convert images to WebP format.',
    author='eboygarcia',
    packages=find_packages(),
    install_requires=[
        'Pillow>=10.0.0',
        'tqdm>=4.0.0',
        'rich>=13.0.0',
        'questionary>=2.0.0',
    ],
    entry_points={
        'console_scripts': [
            'webp-convert=webp_converter.cli:main',
        ],
    },
    python_requires='>=3.7',
)
