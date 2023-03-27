import cv2 as cv
import sys
import numpy as np
import os
from PIL import Image


# print(os.path.abspath(__file__))
line = "--------------------------------"
# Get user supplied values
imageFolderPath = f"input/{sys.argv[1]}"
cascPaths = [
    "./model/haarcascade_frontalcatface.xml",
    "./model/haarcascade_frontalcatface_extended.xml",
    "./model/haarcascade_frontalface_alt.xml",
    "./model/haarcascade_frontalface_alt2.xml",
    "./model/haarcascade_frontalface_alt_tree.xml",
    "./model/haarcascade_profileface.xml",
    "./model/haarcascade_frontalface_default.xml"
]

cascPaths1=[
    "./model/res10_300x300_ssd_iter_140000_fp16.caffemodel"
]


images = os.listdir(imageFolderPath)
loaded_images = []
print(line)
for image in images:
    print(image)
    if image.lower().endswith(('.png', '.jpg', '.jpeg', '.tiff', '.bmp', '.gif')):
        loaded_images.append(f"{imageFolderPath}/{image}")
        
print(line)
# Create the haar cascade
count = 0
for cascpath in cascPaths:
    print(cascpath)
    faceCascade = cv.CascadeClassifier(cascpath)
    # Read the image
    for imagePath in loaded_images:
        if count >0:
            print("name change")
            imagePath = imagePath.replace("input","output")
        # print(imagePath)
        image = cv.imread(imagePath)
        gray = cv.cvtColor(image, cv.COLOR_BGR2GRAY)

        faces = faceCascade.detectMultiScale(
            gray,
            scaleFactor=1.1,
            minNeighbors=1,
            minSize=(20, 20)
        )
        print(f"\nFound {len(faces)} faces!")

        for (x, y, w, h) in faces:
            print(f"{x}, {y}, {w}, {h}")
            ROI = image[y:y+h, x:x+w]
            blur = cv.GaussianBlur(ROI, (51,51), 0) 
            image[y:y+h, x:x+w] = blur
        print(imagePath)
        outputPath = imagePath.replace("input", "output")
        print(outputPath)
        if os.path.exists(f"output/{sys.argv[1]}") == False:
            os.mkdir(f"output/{sys.argv[1]}")
        cv.imwrite(f"{outputPath}", image)
        # cv.imshow("Faces found", image)
        # cv.waitKey(0)
    count += 1