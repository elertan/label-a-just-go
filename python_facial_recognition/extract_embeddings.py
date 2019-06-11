from imutils import paths
from sklearn.preprocessing import LabelEncoder
from sklearn.svm import SVC
import numpy as np
import argparse
import imutils
import pickle
import cv2
import os


# config
detectie = "face_detection_model"
embeddings = "output/embeddings.pickle"
dataset = "dataset"
e_model = "openface_nn4.small2.v1.t7"
recognizer_i ="output/recognizer.pickle"
le_i ="output/le.pickle"
confidence_conf = 0.8

def vectorenCreate(detectie,embeddings,dataset,e_model,confidence_conf):
	# Inladen Caffemodel
	print("Bestanden worden ingeladen en pyTorch")
	protoPath = os.path.sep.join([detectie, "deploy.prototxt"])
	modelPath = os.path.sep.join([detectie,
		"res10_300x300_ssd_iter_140000.caffemodel"])
	detectie = cv2.dnn.readNetFromCaffe(protoPath, modelPath)
	# Inladen premade PyTorch
	embedder = cv2.dnn.readNetFromTorch(e_model)
	# Inladen afbeeldingen
	total = 0
	imagePaths = list(paths.list_images(dataset))
	vectoren = []
	Geaccepteerden = []

	#inladen per afbeelding
	for (i, imagePath) in enumerate(imagePaths):
		print("Inladen afbeelding {}/{}".format(i + 1,
			len(imagePaths)))
		name = imagePath.split(os.path.sep)[-2]
		image = cv2.imread(imagePath)
		image = imutils.resize(image, width=600)
		(h, w) = image.shape[:2]
		imageBlob = cv2.dnn.blobFromImage(cv2.resize(image, (300, 300)), 1.0, (300, 300),(104.0, 177.0, 123.0), swapRB=False, crop=False)
		detectie.setInput(imageBlob)
		detections = detectie.forward()
		if len(detections) > 0:
			i = np.argmax(detections[0, 0, :, 2])
			confidence = detections[0, 0, i, 2]
			if confidence > confidence_conf:
				box = detections[0, 0, i, 3:7] * np.array([w, h, w, h])
				(startX, startY, endX, endY) = box.astype("int")
				face = image[startY:endY, startX:endX]
				(fH, fW) = face.shape[:2]
				if fW < 20 or fH < 20:
					continue

				faceBlob = cv2.dnn.blobFromImage(face, 1.0 / 255,
					(96, 96), (0, 0, 0), swapRB=True, crop=False)
				embedder.setInput(faceBlob)
				vec = embedder.forward()
				Geaccepteerden.append(name)
				vectoren.append(vec.flatten())
				total += 1

	data = {"embeddings": vectoren, "names": Geaccepteerden}
	f = open(embeddings, "wb")
	f.write(pickle.dumps(data))
	f.close()

def train(embeddings,recognizer_i,le_i):
	print("Vectoren worden ingeladen")
	data = pickle.loads(open(embeddings, "rb").read())
	le = LabelEncoder()
	labels = le.fit_transform(data["names"])
	print("Model trainen")
	recognizer = SVC(C=1.0, kernel="linear", probability=True)
	recognizer.fit(data["embeddings"], labels)
	f = open(recognizer_i, "wb")
	f.write(pickle.dumps(recognizer))
	f.close()
	f = open(le_i, "wb")
	f.write(pickle.dumps(le))
	f.close()

vectorenCreate(detectie,embeddings,dataset,e_model,confidence_conf)
train(embeddings,recognizer_i,le_i)