
import sys
import numpy as np
import cv2 
import os
import imutils
# import caffe

imageFolderPath = f"input/{sys.argv[1]}"
prototxt = "./model/deploy.prototxt" 
weights = "./model/res10_300x300_ssd_iter_140000_fp16.caffemodel" 
# prototxt = "./model/vgg_face_caffe/VGG_FACE_deploy.prototxt"
# weights = "./model/vgg_face_caffe/VGG_FACE.caffemodel"
net = cv2.dnn.readNet(weights,prototxt,'test')
line = "--------------------------------"
# image = "./input/test/IMG_0332.jpeg"
# image = cv2.imread(image)
# 이미지 resize



images = os.listdir(imageFolderPath)
loaded_images = []
print(line)
for image in images:
    print(image)
    if image.lower().endswith(('.png', '.jpg', '.jpeg', '.tiff', '.bmp', '.gif')):
        loaded_images.append(f"{imageFolderPath}/{image}")
for imagePath in loaded_images:
    nums = 0
    image = cv2.imread(imagePath)
    # img = caffe.io.load_image(imagePath)
    # img = img[:,:,::-1]*255.0 # convert RGB->BGR
    # avg = np.array([93.5940, 104.7624, 129.1863])  # BGR mean values
    # imgage = img - avg # subtract mean (numpy takes care of dimensions :)
    # print(image.size)
    # image = imutils.resize(image, width=4000)
    blob = cv2.dnn.blobFromImage(image, 4, (300, 300), (104, 177, 123))
    net.setInput(blob)
    detect = net.forward()
    (h, w) = image.shape[:2]
    detect = detect[0, 0, :, :]
    for i in range(detect.shape[0]):
        confidence = detect[i, 2]
        print(confidence)
        if confidence < 0.125 or nums > 100:
            break
        x1 = int(detect[i, 3] * w)
        y1 = int(detect[i, 4] * h)
        x2 = int(detect[i, 5] * w)
        y2 = int(detect[i, 6] * h)
        topLeft = (x1, y1)
        bottomRight = (x2, y2)
        x, y = topLeft[0], topLeft[1]
        ww, hh = bottomRight[0] - topLeft[0], bottomRight[1] - topLeft[1]
        # cv2.rectangle(image, (x1, y1), (x2, y2), (0, 255, 0))
        # label = 'Face: %4.3f' % confidence
        # cv2.putText(image, label, (x1, y1 - 1), cv2.FONT_HERSHEY_SIMPLEX, 0.8, (0, 255, 0), 1, cv2.LINE_AA)
        # print(detect)
        ROI = image[y:y+hh, x:x+ww]
        try:
            blur = cv2.GaussianBlur(ROI, (51,51), 0) 
            image[y:y+hh, x:x+ww] = blur
        except:
            print(nums)
            nums +=1
            continue
    # cv2.imshow('image', image)
    print(imagePath)
    outputPath = imagePath.replace("input", "output")
    print(outputPath)
    if os.path.exists(f"output/{sys.argv[1]}") == False:
        os.mkdir(f"output/{sys.argv[1]}")
    cv2.imwrite(f"{outputPath}", image)
    


