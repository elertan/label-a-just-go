import cv2
import tensorflow as tf

CATEGORIES = ["3a10174f-a213-464c-9ee9-39c6af7999fc","3ba32db6-4468-43c8-997d-db2f16a00659","6b8c5732-a563-428d-ad56-157a154367da","6b8c5732-a563-428d-ad56-157a154367dd","87ccfcc9-002d-427A-B07B-3f00660fc8ed","7077eda5-5545-4793-853b-98a5e6f6e310"]  # will use this to convert prediction num to string value


def prepare(filepath):
    IMG_SIZE = 100  # 50 in txt-based
    img_array = cv2.imread(filepath, cv2.IMREAD_GRAYSCALE)  # read in the image, convert to grayscale
    new_array = cv2.resize(img_array, (IMG_SIZE, IMG_SIZE))  # resize image to match model's expected sizing
    return new_array.reshape(-1, IMG_SIZE, IMG_SIZE, 1)
model = tf.keras.models.load_model("64x3-CNN.model")
prediction = model.predict([prepare('P3_Ambient.pgm')])
print(prediction)
print(CATEGORIES[int(prediction[0][0])])