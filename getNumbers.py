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
    y=100
    x=220
    h=300
    w=500
    crop = image[y:y+h, x:x+w]
    cv2.imwrite(path, crop)



#gets number from croped image
def getNumberFromImage():
    image = openImage()
    myConfig='--psm 11 --oem 3 -c tessedit_char_whitelist=0123456789'
    text = pytesseract.image_to_string(image, config=myConfig)
    text = re.sub("[^0-9]", "", text)
    return text


def replaceBlackByWhite():
    img = openImage()
    black_pixels = np.where(
    (img[:, :, 0] == 0) & 
    (img[:, :, 1] == 0) & 
    (img[:, :, 2] == 0)
    )
    # set those pixels to white
    img[black_pixels] = [255, 255, 255]
    cv2.imwrite(path, img)



def addEntryToCsv(num):
    with open('raw_data.csv', mode='a') as file:
        writer = csv.writer(file)
        writer.writerow([num, Timestamp.now()])




download_image()
cropImage()
#replaceBlackByWhite() 
num = getNumberFromImage()
print(num)
addEntryToCsv(num)




