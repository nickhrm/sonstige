from numbers import Number
from operator import iadd
from sqlite3 import Timestamp
from tokenize import String
from typing import Text
from numpy import add
import requests
import pytesseract
import cv2
import csv
import re
import numpy as np



path = 'image.jpeg'

def openImage():
    return cv2.imread(path)



def download_image():
    image_url = 'https://web.hochschulsport-hannover.de/campusfit/auslastungsgrafik'
    # HTTP GET Request zum Abrufen des Bildes
    response = requests.get(image_url)
    # Überprüfen, ob die Anfrage erfolgreich war
    if response.status_code == 200:
        # Bild im binären Modus öffnen und speichern
        print(f'Status 200')
        with open(path, 'wb') as f:
            f.write(response.content)
    else:
        print(f'Fehler beim Abrufen des Bildes: {response.status_code}')



#takes image as input and crops it so only the number is visible
def cropImage():
    image = openImage()
    y=120
    x=220
    h=290
    w=600
    crop = image[y:y+h, x:x+w]
    cv2.imwrite(path, crop)


def removeAllButGreen():
    original_img = openImage()
    original_img[np.where((original_img!=[64,196,141]).all(axis=2))] = [255,255,255]
    cv2.imwrite(path, original_img)


def rescale():
    image = openImage()
    image = cv2.resize(image, None, fx=1.2, fy=1.2, interpolation=cv2.INTER_CUBIC)
    cv2.imwrite(path, image)

def blurImage():
    image = openImage()
    image = (cv2.bilateralFilter(image, 5, 75, 75), 0, 255, cv2.THRESH_BINARY + cv2.THRESH_OTSU)[1]
    cv2.imwrite(path, image)


def makeImageGrayScale():
    image = openImage()
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    cv2.imwrite(path, gray)

#gets number from croped image
def getNumberFromImage():
    image = openImage()
    myConfig='--psm 13 --oem 1 -c tessedit_char_whitelist=0123456789'
    text = pytesseract.image_to_string(image, config=myConfig)
    text = re.sub("[^0-9]", "", text)
    return text

#add white border to image so the numer is smaller
def expandImage():
    image = openImage()
    top = 300
    bottom = 400
    left = 300
    right = 400

    image = cv2.copyMakeBorder(image, top, bottom, left, right, cv2.BORDER_CONSTANT, value=[255,255,255])
    cv2.imwrite(path, image)

def addEntryToCsv(num):
    if num == "":
        return
    with open('raw_data.csv', mode='a') as file:
        writer = csv.writer(file)
        writer.writerow([Timestamp.now(),num])




download_image()
removeAllButGreen()

cropImage()
makeImageGrayScale()
expandImage()
rescale()
num = getNumberFromImage()
print(num)
addEntryToCsv(num)




