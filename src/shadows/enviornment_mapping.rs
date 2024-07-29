/// shading from environment lighting
// informally named image-based lighting

// how to use it to shade a point(without shadows)

// general solution - Monte carlo integratipon
// numerical
// large amount of samples required

// problem - can be slow
// in general, sampling is not preferred in shaders
// can we avoid sampling ?

// BRDF satifies the accuracy condition in any case
// we can safely take the lighting term out

// The split Sum(split integral) Approximation
// That's why we call it split sum

// Finally, completely avoided sampling
// very fast and almost identical results

// shadow from environment lighting
// in genreal, very deifficult for real-time rendering
// industrial solution
// Generate one(or a little bit more) shadows from the brightness light source
// precomputed radiance transfer

// RTRT : path tracing + de-noisying
