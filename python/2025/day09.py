from matplotlib import pyplot as plt
import numpy as np


def calculate_area(x1, y1, x2, y2): 
    return (abs(x1-x2)+1)*(abs(y1-y2)+1)


def contains(rect_x1, rect_y1, rect_x2, rect_y2, x,y):
    rect_x1, rect_x2 = sorted([x1, x2])
    rect_y1, rect_y2 = sorted([y1, y2])
    return (rect_x1 < x and x < rect_x2) and (rect_y1 < y and y < rect_y2)



coords = np.loadtxt('data/day09.input', delimiter=',')
x = [c[0] for c in coords];
y = [c[1] for c in coords];


x_distances = [[(x[i] - x[i+1])**2, i] for i in range(len(coords)-1)]

sorted_x_distances = sorted(x_distances, reverse=True)

outlier_coord_idx_0 = sorted_x_distances[0][1]
outlier_coord_idx_1 = sorted_x_distances[1][1]

outlier_x = [coords[outlier_coord_idx_0][0], coords[outlier_coord_idx_1][0]]
outlier_y = [coords[outlier_coord_idx_0][1], coords[outlier_coord_idx_1][1]]

[y_out_min, y_out_max] = sorted(outlier_y)

pt2 = 0
areas = []
for i in range(len(coords)):
    for j in range(i):
        yi = coords[i][1]
        yj = coords[j][1]
        [ymin, ymax] = sorted([yi, yj])
        if ymax <= y_out_min or ymin >= y_out_max:
            area = calculate_area(coords[i][0], coords[i][1], coords[j][0], coords[j][1])
            areas.append([area, i, j])



areas = sorted(areas, reverse=True)
print(len(areas))


pt2 = 0
box_coords = []
for [area, i, j] in areas:
    to_continue = False
    for c in coords:
        x1, y1 = coords[i]
        x2, y2 = coords[j]
        xx, yy = c
        if contains(x1, y1, x2, y2, xx, yy): 
            to_continue = True
            break
    if to_continue: continue 
    pt2 = area
    box_coords = [[x1, y1], [x2, y2]]
    break
print(area)



# plt.plot(outlier_x, outlier_y, 'o')
# plt.hlines(outlier_y[0], xmin=0, xmax=100000, color='r')
# plt.hlines(outlier_y[1], xmin=0, xmax=100000, color='r')
plt.plot(x, y)
plt.plot(box_coords[0][0], box_coords[0][1], 'o')
plt.plot(box_coords[1][0], box_coords[1][1], 'o')
plt.show()
