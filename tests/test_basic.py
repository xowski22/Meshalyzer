import meshalyzer

def test_version():
    assert meshalyzer.version() == "0.1.0"

if __name__ == "__main__":
    print(f"Meshalyzer version: {meshalyzer.version()}")