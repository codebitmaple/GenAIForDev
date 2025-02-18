def encode_image(image_path):
    """
    Encodes an image file to a base64 string.

    Args:
        image_path (str): Path to the image file.

    Returns:
        str: Base64-encoded string of the image.

    Raises:
        FileNotFoundError: If the image file does not exist.
    """
    if not os.path.isfile(image_path):
        raise FileNotFoundError(f"Image file not found at path: {image_path}")

    with open(image_path, "rb") as image_file:
        encoded_string = base64.b64encode(image_file.read()).decode("utf-8")
    return encoded_string
