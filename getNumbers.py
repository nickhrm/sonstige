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
    y=0
    x=0
    h=2000
    w=1000
    crop = image[y:y+h, x:x+w]
    cv2.imwrite(path, crop)



#gets number from croped image
def getNumberFromImage():
    image = openImage()
    myConfig='--psm 11 --oem 2 -c tessedit_char_whitelist=0123456789'
    text = pytesseract.image_to_string(image, config=myConfig)
    text = re.sub("[^0-9]", "", text)
    return text


def removeAllButGreen():
    original_img = openImage()
    original_img[np.where((original_img!=[64,196,141]).all(axis=2))] = [255,0,255]
    cv2.imwrite(path, original_img)

    
def makeImageGrayScale():
    im_gray = cv2.imread(path, cv2.IMREAD_GRAYSCALE)
    (thresh, im_bw) = cv2.threshold(im_gray, 128, 255, cv2.THRESH_BINARY | cv2.THRESH_OTSU)
    im_bw = cv2.threshold(im_gray, thresh, 255, cv2.THRESH_BINARY)[1]
    cv2.imwrite(path, im_bw)



def addEntryToCsv(num):
    with open('raw_data.csv', mode='a') as file:
        writer = csv.writer(file)
        writer.writerow([num, Timestamp.now()])




download_image()
removeAllButGreen()
makeImageGrayScale()
cropImage()
num = getNumberFromImage()
print(num)
addEntryToCsv(num)




