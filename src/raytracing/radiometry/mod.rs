// Radiant Flux(Power)
// Q [J = Joule]
// flux p = dQ / dt = W = Watt  <==> lm = lumen

// Radiant intensity
// intensity power per unit solid angle
// I(w) = dP / dw = W / sr = lm / sr = cd = candela
//
// Solid angle?
// angle:  a = l / r
// solid angle:  o =  A / r^2      s = 4 * PI * r^2
//
// Differential Solid angles
// dA = r^2 * sina da db
// I = P / (4 * PI)

// Irradiance: total power received by area dA
// power per unit area
// E(x) = dO / dA = W / m^2 = lm / m^2 = lux

// Lambert's Cosine Law
// E(x) = (dO / dA) * cos(a)

// Radiance: Power received by area dA from "direction" dW
// Definition: Power per unit solid angle, per projected unit area
// L(p, w) = d^2O(p, w) / (dw * dA * cos(a))

// Irradiance vs Radiance
// dE(p,w) =  Li(p, w)cos dW

// Bidirectional Reflectance Distribute Function(BRDF)
// irradiance dE
//
//
//
//

// The rendering Equation
// Lo(p, wo) = Le(p, wo) + S Li(p, wi) fr(p, wi, wo)(n * wi) dWi
// this is

// Path tracing N = 1 Distributed Path tracing N != 1

// Biased vs Unbiased Monte Carlo Estimators
// * an unbiased Monte Carlo technique does not have any systematic error
//
// * biased, one special case, the expected avalue converges to the correct value as infinite #samples
// are used - consistent

// Bidirectional Path Tracing (BDPT)
// Suitable if the light transport is complex on the light's side
// Difficult to implement & quite slow

// Metropolis Light Transport (MLT)
// A Markov Chain Monte Carlo (MCMC) application
// - Jumping from the current sample to the next with some PDF
// - very good at locally exploring difficult light paths
//
// cons:
// - difficult to estimate the convergence rate
// does not guarantee equal convergence rate per pixel
// so, usually produces 'dirty results'

// Biased
//

// Photon Mapping
// A biased approach & a two-staged method
// very good at rendering costics
//
// biased but consistent

// Vertex Connection and Merging

// Instant Radiosity(IR)

// Paricipating Media: fog, cloud

// noise
// perlin noise
// voronoi noise
//
