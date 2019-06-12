

from imutils.video import VideoStream
import face_recognition
import argparse
import imutils
import pickle
import time
import cv2
import os
import statistics 
import json
from statistics import mode 

data_i = []
speed = 5
framespeed = 1
checklist = []
guesslist = []

print("Inladen vectoren...")
data = pickle.loads(open("encodings.pickle", "rb").read())

print("Video starten...")
vs = VideoStream(src=1,framerate=framespeed).start()
writer = None
time.sleep(4.0)
while True:
	frame = vs.read()
	rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
	rgb = imutils.resize(frame, width=360)
	r = frame.shape[1] / float(rgb.shape[1])
	boxes = face_recognition.face_locations(rgb,
		model="hog")
	encodings = face_recognition.face_encodings(rgb, boxes)
	names = []
	for encoding in encodings:

		matches = face_recognition.compare_faces(data["encodings"],
			encoding)
		name = "Unknown"

		if True in matches:
		
			matchedIdxs = [i for (i, b) in enumerate(matches) if b]
			counts = {}

			for i in matchedIdxs:
				name = data["names"][i]
				counts[name] = counts.get(name, 0) + 1

			name = max(counts, key=counts.get)
	
		names.append(name)

	for ((top, right, bottom, left), name) in zip(boxes, names):
		top = int(top * r)
		right = int(right * r)
		bottom = int(bottom * r)
		left = int(left * r)
		if(name == "unknown"):
			y = top - 10 if top - 10 > 10 else top + 10
			cv2.rectangle(frame, (left, top), (right, bottom),(0, 0,255), 2)
		if(name != "unknown" and name not in checklist ):
			guesslist.append(name)
			y = top - 10 if top - 10 > 10 else top + 10
			cv2.rectangle(frame, (left, top), (right, bottom),(0, 255,0), 2)
			cv2.putText(frame, name, (left, y),cv2.FONT_HERSHEY_SIMPLEX, 0.45, (0, 255, 0), 2)
				
			if(len(guesslist) > speed):
				result=mode(guesslist)
				checklist.append(result)	
				data_i.append({
				'uuid':result,
				'left':left,
				'top':top,
				'right':right,
				'bottom':bottom
				})
				cv2.imwrite(name+".jpg",frame[top:bottom,left:right])
				with open('data.json',"w") as outfile:
					json.dump(data_i,outfile)
				print("JSON- added")
				guesslist = []


	cv2.imshow("Gezichts_herkenning_evenement", frame)
	key = cv2.waitKey(1) & 0xFF
	if key == ord("q"):
		break

cv2.destroyAllWindows()
vs.stop()

