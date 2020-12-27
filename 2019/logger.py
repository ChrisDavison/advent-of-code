import logging

def setup_logger(name, level=logging.INFO):
    fmt = '%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    logger = logging.getLogger(name)
    logging.basicConfig(level=level, style="{")
    logger.setLevel(level)
    console_handler = logging.StreamHandler()
    console_handler.setLevel(level)
    formatter = logging.Formatter(fmt)
    console_handler.setFormatter(formatter)
    logger.addHandler(console_handler)

