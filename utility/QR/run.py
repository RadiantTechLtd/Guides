import qrcode

img = qrcode.make("https://github.com/RadiantTechLtd/Guides/")
type(img)  # qrcode.image.pil.PilImage
img.save("some_file.png")
