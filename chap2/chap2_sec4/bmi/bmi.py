height_cm = float(input("身長(cm)は？: "))
weight = float(input("体重(kg)は？: "))

height = height_cm / 100.0
bmi = weight / (height ** 2)
print("BMI={:.1f}".format(bmi))

if bmi < 18.5:
    print("低体重")
elif bmi < 25:
    print("普通体重")
elif bmi < 30:
    print("肥満1度")
elif bmi < 35:
    print("肥満2度")
elif bmi < 40:
    print("肥満3度")
else:
    print("肥満4度")
