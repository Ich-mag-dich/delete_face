
import sys
import numpy as np
import cv2 
import os
import tensorflow as tf
import imutils
import keras
import keras_vggface 
from keras_vggface.vggface import VGGFace
from keras_vggface import utils
import mtcnn 
import matplotlib as mpl
from keras.preprocessing import image
# from keras.engine import  Model
from keras.layers import Flatten, Dense, Input
# from keras.utils.data_utils import get_file
# import keras_vgqface.utils

imageFolderPath = f"input/{sys.argv[1]}"
# prototxt = "./model/deploy.prototxt" 
# weights = "./model/res10_300x300_ssd_iter_140000_fp16.caffemodel" 
# net = cv2.dnn.readNet(weights,prototxt)
line = "--------------------------------"
# image = "./input/test/IMG_0332.jpeg"
# image = cv2.imread(image)
# 이미지 resize
print(tf.__version__)
# vggface_resnet = VGGFace(model='resnet50')
vggface_resnet = VGGFace(model='vgg16')

nb_class = 2
hidden_dim = 512

# vgg_model = VGGFace(include_top=False)
# last_layer = vgg_model.get_layer('pool5').output
# q = Flatten(name='flatten')(last_layer)
# q = Dense(hidden_dim, activation='relu', name='fc6')(q)
# q = Dense(hidden_dim, activation='relu', name='fc7')(q)
# out = Dense(nb_class, activation='softmax', name='fc8')(q)
# custom_vgg_model = keras.Model(vgg_model.input, out)

images = os.listdir(imageFolderPath)
loaded_images = []
print(line)
for image in images:
    print(image)
    if image.lower().endswith(('.png', '.jpg', '.jpeg', '.tiff', '.bmp', '.gif')):
        loaded_images.append(f"{imageFolderPath}/{image}")
face_detector = mtcnn.MTCNN(
    min_face_size=20
)
for imagePath in loaded_images:
    nums = 0
    img = cv2.imread(imagePath)
    # img = cv2.cvtColor(cv2.imread(imagePath), cv2.COLOR_BGR2RGB)
    # img = tf.keras.utils.load_img(imagePath)
    # img = imutils.resize(img, width=224, height=224)
    # img = image.load_img(imagePath, target_size=(224,224))
    # z = tf.keras.utils.img_to_array(img)
    # z = np.expand_dims(z, axis=0)
    # z = utils.preprocess_input(z, version=2) # or version=2
    # preds = vggface_resnet.predict(x)
    # print('Predicted:', utils.decode_predictions(preds))
    # print(x)
    # x = vggface_resnet
    # print(z)
    # print(vggface_resnet.predict(img))
    face_roi = face_detector.detect_faces(img)
    # x,y,w,h = face_roi[0]['box']
    for i in face_roi:
        x,y,w,h = i['box']
        print(i['confidence'])
        ROI = img[y:y+h, x:x+w]
        blur = cv2.GaussianBlur(ROI, (51,51), 0) 
        img[y:y+h, x:x+w] = blur 
    print(imagePath)
    outputPath = imagePath.replace("input", "output")
    print(outputPath)
    if os.path.exists(f"output/{sys.argv[1]}") == False:
        os.mkdir(f"output/{sys.argv[1]}")
    cv2.imwrite(f"{outputPath}", img)


