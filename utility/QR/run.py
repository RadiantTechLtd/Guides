import qrcode


message = "https://github.com/RadiantTechLtd/Guides/"
filename = "radiant_tech.png"

img = qrcode.make(message)
img.save(filename)
