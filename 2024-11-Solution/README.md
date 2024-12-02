

Given a unit square with vertices: $(0, 0), (0, 1), (1, 0), (1, 1)$:

For a triangle bounded: $0 < y < x$ from $0 < x < 1/2$

$A_{circles} = \frac{\pi}{4}(x^2 + y^2) + \frac{\pi}{4}((1-x)^2 + y^2)$

$A_{intersection} = (x^2 + y^2)\mathrm{cos}^{-1}(\frac{x}{\sqrt{x^2+y^2}}) + ((1-x)^2 + y^2)\mathrm{cos}^{-1}(\frac{1-x}{\sqrt{(1-x)^2+y^2}}) - y$

$Solution = 8*\int_0^{1/2} \int_0^x (A_{circles} - A_{intersection}) \mathrm{d}y\space\mathrm{d}x$

$Area_{average} = 0.491407578838 $
