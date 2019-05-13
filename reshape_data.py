import numpy as np
import matplotlib.pyplot as plt
import os
import cv2
import pickle
import random

training_data = []
def create_training_data():
	datadir = "img/"
	categories = ["3a10174f-a213-464c-9ee9-39c6af7999fc","3ba32db6-4468-43c8-997d-db2f16a00659","6b8c5732-a563-428d-ad56-157a154367da","6b8c5732-a563-428d-ad56-157a154367dd","87ccfcc9-002d-427A-B07B-3f00660fc8ed","7077eda5-5545-4793-853b-98a5e6f6e310"]
	IMG_SIZE = 50

	for x in categories:
		path = os.path.join(datadir,x)
		class_num = categories.index(x)
		for img in os.listdir(path):
			img_array = cv2.imread(os.path.join(path,img) ,cv2.IMREAD_GRAYSCALE)
			new_array = cv2.resize(img_array, (IMG_SIZE, IMG_SIZE))
			training_data.append([new_array , class_num])
			#plt.imshow(new_array)
			#plt.show()
	random.shuffle(training_data)
	X = []
	Y = []
	for features, label in training_data:
		X.append(features)
		Y.append(label)
	X = np.array(X).reshape(-1, IMG_SIZE, IMG_SIZE, 1)
	pickle_out = open("X.pickle","wb")
	pickle.dump(X, pickle_out)
	pickle_out.close()
	pickle_out = open("Y.pickle","wb")
	pickle.dump(Y, pickle_out)
	pickle_out.close()
	pickle_in = open("X.pickle","rb")
	X = pickle.load(pickle_in)
	pickle_in = open("Y.pickle","rb")
	Y = pickle.load(pickle_in)
		
create_training_data()
print(len(training_data))