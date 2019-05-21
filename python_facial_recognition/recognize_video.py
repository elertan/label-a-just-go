# USAGE
# python recognize_video.py --detector face_detection_model \
#	--embedding-model openface_nn4.small2.v1.t7 \
#	--recognizer output/recognizer.pickle \
#	--le output/le.pickle

# import the necessary packages
from imutils.video import VideoStream
from imutils.video import FPS
import numpy as np
import argparse
import imutils
import pickle
import time
import cv2
import os
import statistics 
import json
from statistics import mode 

# config
dectector="face_detection_model"
embedding_model="openface_nn4.small2.v1.t7"
recognizer="output/recognizer.pickle"
le="output/le.pickle"
confidence_c= 0.8
data = []
speed = 20
checklist = []

protoPath = os.path.sep.join([dectector, "deploy.prototxt"])
modelPath = os.path.sep.join([dectector,
	"res10_300x300_ssd_iter_140000.caffemodel"])
detector = cv2.dnn.readNetFromCaffe(protoPath, modelPath)
embedder = cv2.dnn.readNetFromTorch(embedding_model)

recognizer = pickle.loads(open(recognizer, "rb").read())
le = pickle.loads(open(le, "rb").read())


vs = VideoStream(src=0).start()
time.sleep(2.0)
print("The cam is filming")
guesslist = []

fps = FPS().start()

# loop over frames from the video file stream
while True:
	# grab the frame from the threaded video stream
	frame = vs.read()


	frame = imutils.resize(frame, width=600)
	(h, w) = frame.shape[:2]

	# construct a blob from the image
	imageBlob = cv2.dnn.blobFromImage(
		cv2.resize(frame, (300, 300)), 1.0, (300, 300),
		(104.0, 177.0, 123.0), swapRB=False, crop=False)

	# apply OpenCV's deep learning-based face detector to localize
	# faces in the input image
	detector.setInput(imageBlob)
	detections = detector.forward()

	# loop over the detections
	for i in range(0, detections.shape[2]):
		# extract the confidence (i.e., probability) associated with
		# the prediction
		confidence = detections[0, 0, i, 2]

		# filter out weak detections
		if confidence > confidence_c:
			# compute the (x, y)-coordinates of the bounding box for
			# the face
			box = detections[0, 0, i, 3:7] * np.array([w, h, w, h])
			(startX, startY, endX, endY) = box.astype("int")

			# extract the face ROI
			face = frame[startY:endY, startX:endX]
			(fH, fW) = face.shape[:2]

			# ensure the face width and height are sufficiently large
			if fW < 20 or fH < 20:
				continue

			# construct a blob for the face ROI, then pass the blob
			# through our face embedding model to obtain the 128-d
			# quantification of the face
			faceBlob = cv2.dnn.blobFromImage(face, 1.0 / 255,
				(96, 96), (0, 0, 0), swapRB=True, crop=False)
			embedder.setInput(faceBlob)
			vec = embedder.forward()

			# perform classification to recognize the face
			preds = recognizer.predict_proba(vec)[0]
			j = np.argmax(preds)
			proba = preds[j]
			name = le.classes_[j]

			# draw the bounding box of the face along with the
			# associated probability
			text = "{}: {:.2f}%".format(name, proba * 100)
			if(name == "unknown"):
				y = startY - 10 if startY - 10 > 10 else startY + 10
				cv2.rectangle(frame, (startX, startY), (endX, endY),
					(0, 0,255), 2)
			if(name != "unknown" and name not in checklist ):
				
				guesslist.append(name)
				y = startY - 10 if startY - 10 > 10 else startY + 10
				cv2.rectangle(frame, (startX, startY), (endX, endY),
					(0, 255,0), 2)
				cv2.putText(frame, name, (startX, y),
					cv2.FONT_HERSHEY_SIMPLEX, 0.45, (0, 255, 0), 2)
				
				if(len(guesslist) > speed):
					result=mode(guesslist)
					checklist.append(result)
					
					data.append({
					'uuid':result,
					'confidence':confidence.item(),
					'startX':startX.item(),
					'startY':startY.item(),
					'endX':endX.item(),
					'endY':endY.item()
					})
					print(frame)
					cv2.imwrite(name+".jpg",frame[startY:endY,startX:endX])
					with open('data.json',"w") as outfile:
						json.dump(data,outfile)
					print("JSON- added")
					guesslist.clear()
					
				

	# update the FPS counter
	fps.update()

	# show the output frame
	cv2.imshow("Frame", frame)
	key = cv2.waitKey(1) & 0xFF

	# if the `q` key was pressed, break from the loop
	if key == ord("q"):
		break
# stop the timer and display FPS information
fps.stop()
print("Elasped time: {:.2f}".format(fps.elapsed()))
print("Approx. FPS: {:.2f}".format(fps.fps()))

# do a bit of cleanup
cv2.destroyAllWindows()
vs.stop()
