import matplotlib.pyplot as plt
import numpy as np
from tqdm import tqdm
from matplotlib.patches import Circle, Rectangle
import yaml

with open('config.yaml', 'r') as file:
    CONFIG = yaml.safe_load(file)

# CONFIG PARAMETERS
RUN_SIM = CONFIG['simulation']['run_sim']
X_LIMITS = CONFIG['graph_params']['x_limits']
Y_LIMITS = CONFIG['graph_params']['y_limits']
X_LIMIT_LOW, X_LIMIT_HIGH = X_LIMITS['low'], X_LIMITS['high']
Y_LIMIT_LOW, Y_LIMIT_HIGH = Y_LIMITS['low'], Y_LIMITS['high']
SQUARE_WIDTH, SQUARE_HEIGHT = X_LIMIT_HIGH - X_LIMIT_LOW, Y_LIMIT_HIGH - Y_LIMIT_LOW
# TEST SAMPLE POINTS
SAMPLE_X_POINT = CONFIG['graph_params']['x_point']
SAMPLE_Y_POINT = CONFIG['graph_params']['y_point']

if SQUARE_WIDTH != SQUARE_HEIGHT:
    raise Exception("Not well formed square.")

def create_prediction_functions():
    slope1 = (Y_LIMIT_HIGH - Y_LIMIT_LOW) / (X_LIMIT_HIGH - X_LIMIT_LOW)
    slope2 = (Y_LIMIT_LOW - Y_LIMIT_HIGH) / (X_LIMIT_HIGH - X_LIMIT_LOW)
    coef1 = Y_LIMIT_LOW - slope1 * X_LIMIT_LOW
    coef2 = Y_LIMIT_LOW - slope2 * X_LIMIT_HIGH
    return {"pos_diag": (slope1, coef1), "neg_diag": (slope2, coef2)}

DIAGONALS = create_prediction_functions()

def prediction_fn(x_actual:float, line_info):
    slope, coef = line_info
    return slope*x_actual+coef

def is_above(y_actual: float, x_actual: float, pos_or_neg:str):
    if pos_or_neg not in ["pos_diag","neg_diag"]: raise Exception(f"Invalid value for pos_or_neg: {pos_or_neg}. Requires 'pos_diag' or 'neg_diag'.")
    y_pred = prediction_fn(x_actual, DIAGONALS[pos_or_neg])
    return y_actual > y_pred

# 	### 2 ###
#   1 ##### 3
#.  ### 4 ###
def find_closest_line(x, y):
    if is_above(y, x, "pos_diag"): # 1 or 2
        if is_above(y, x, "neg_diag"): return 2
        else: return 1
    else: # 3 or 4
        if is_above(y, x, "neg_diag"): return 3
        else: return 4

#  -1, 1 to 1,1
#  -1,-1 to 1,-1
def generate_points_on_line(line_no:int, range_tick: float = 0.1):
    ticks = []
    original_val = 0.0 #TODO: needs to fit based on config params
    while original_val <= 1.0:
        original_val += range_tick
        ticks.append(original_val)

    if line_no == 1:
        return [[X_LIMIT_LOW,i] for i in ticks]
    elif line_no == 2:
        return [[i, Y_LIMIT_HIGH] for i in ticks]
    elif line_no == 3:
        return [[X_LIMIT_HIGH, i] for i in ticks]
    else:
        return [[i,Y_LIMIT_LOW] for i in ticks]

def get_radius(x1, y1, x2, y2):
    return np.sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)

def create_circles(x, y, range_line_points):
    circles = []
    for point in range_line_points:
        circle = Circle((point[0], point[1]), radius=get_radius(x,y, point[0], point[1]), edgecolor='red', facecolor='white', fill=False)
        circles.append(circle)
    return circles


def create_all(x, y, range_tick):
    closest_line = find_closest_line(x, y)
    print("CLOSEST LINE", closest_line)
    circle_centers = generate_points_on_line(closest_line, range_tick)
    circles_output = create_circles(x, y, circle_centers)
    return circles_output


def get_line_points(x, y): # in format x_low, y_low, x_high, y_high
    closest_line = find_closest_line(x, y)
    if closest_line == 1: return X_LIMIT_LOW, Y_LIMIT_LOW, X_LIMIT_LOW, Y_LIMIT_HIGH # left boundary
    elif closest_line == 2: return X_LIMIT_LOW, Y_LIMIT_HIGH, X_LIMIT_HIGH, Y_LIMIT_HIGH # -1, 1, 1, 1 # top boundary
    elif closest_line == 3: return X_LIMIT_HIGH, Y_LIMIT_LOW, X_LIMIT_HIGH, Y_LIMIT_HIGH # 1, -1, 1, 1 # right boundary
    else: return X_LIMIT_LOW, Y_LIMIT_LOW, X_LIMIT_HIGH, Y_LIMIT_LOW #-1, -1, 1, -1 # bottom boundary

# variables to find the intersection
# radius1 is distance to first circle center from randomly chosen point
# radius2 is distance to second circle center from randomly chosen point
def get_area_of_intersection(radius1, radius2, d_between_centers: float=X_LIMIT_HIGH-X_LIMIT_LOW) -> float:
    if d_between_centers >= radius1 + radius2: # circles have no overlap
        return 0
    d1 = (radius1**2 - radius2**2 + d_between_centers**2) / (2 * d_between_centers)
    d2 = (radius2**2 - radius1**2 + d_between_centers**2) / (2 * d_between_centers)
    left = radius1**2 * np.arccos(d1/radius1) - d1 * np.sqrt(radius1**2 - d1**2)
    right = radius2**2 * np.arccos(d2/radius2) - d2 * np.sqrt(radius2**2 - d2**2)
    return left + right

def get_area_of_circle(radius):
    return np.pi * (radius ** 2)

def run_area(x, y):
    x_low, y_low, x_high, y_high = get_line_points(x, y)
    radius1 = get_radius(x_low, y_low, x, y)
    radius2 = get_radius(x_high, y_high, x, y)

    circle1_area = get_area_of_circle(radius1)
    circle2_area = get_area_of_circle(radius2)
    filled_in_area = circle1_area/4.0 + circle2_area/4.0
    intersection_area = get_area_of_intersection(radius1, radius2, 1.0)
    return filled_in_area - intersection_area

def test_area_function(x, y):
    return (x **2 + y ** 2) * (np.pi / 4.0 - np.arccos(x / np.sqrt(x**2+ y**2))) + ((1 - x)**2 + y**2)* (np.pi / 4.0 -np.arccos((1 - x) / np.sqrt((1-x)**2+y**2))) + y


fig, ax = plt.subplots()

# BASE RECTANGLE
rectangle = Rectangle((X_LIMIT_LOW, Y_LIMIT_LOW), SQUARE_WIDTH, SQUARE_HEIGHT, edgecolor='blue', alpha=0.2) # base rectangle
ax.add_artist(rectangle)

FINAL_CIRCLES = create_all(SAMPLE_X_POINT, SAMPLE_Y_POINT, 0.01)
for elem in FINAL_CIRCLES:
    ax.add_artist(elem)


### CONFIGURING MATPLOT GRID #####
ax.set_xlim(X_LIMIT_LOW - 0.5, X_LIMIT_HIGH + 0.5)
ax.set_ylim(Y_LIMIT_LOW - 0.5, Y_LIMIT_HIGH + 0.5)
ax.set_aspect('equal')
plt.grid(True)
plt.title('Besides the Point')
plt.show()


circles_area = run_area(SAMPLE_X_POINT, SAMPLE_Y_POINT)
amount_of_square_covered = circles_area / (SQUARE_WIDTH * SQUARE_HEIGHT)

print("X_POINT: ", SAMPLE_X_POINT)
print("Y_POINT: ", SAMPLE_Y_POINT)
print("BOTH CIRCLES AREA (example point): ", circles_area)
print("AREA COVERING SQUARE (for example): ", amount_of_square_covered)


if not RUN_SIM: exit(0)

# SIMULATION TO APPROXIMATE
def simulate_circle_off_square(iterations=100_000_000, square_size=SQUARE_WIDTH * SQUARE_HEIGHT):
    sum_area = 0
    for _ in tqdm(range(iterations), desc="Simulating", unit="iteration"):
        # Generate two points (x1, y1) and (x2, y2) within the square
        x1, y1 = np.random.uniform(0, 1, 2)

        # Calculate the circle's area based on the random points
        area = run_area(x1, y1) / square_size
        sum_area += area

    return sum_area / iterations

print("Average probability: ", simulate_circle_off_square())

