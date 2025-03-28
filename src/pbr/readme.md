# Physically Based Shading
---

< Let the form of an object be what it may, --light, shade, and perspective will always make it beautiful. -- John Constable

In this chapter we cover the various aspects of physically based shading. We start with a
description of the physics of light-matter interactions, and show how these physics connect
to the shading process. and then dedicated to the building blocks used to construct physically
based shading models, and the models themselves-covering a broad variety of material types.
how materials blender together, and we cover filtering methods for avoiding aliasing and
preserving surface appearance.

### Physics of Light
The interactions of light and matter form the foundation of physically based shading. To understand
these interactions, it helps to have a basic understanding of the nature of light.

so what is light?

Light, an electromagnetic transverse wave. The electric and magnetic field vectors oscillate at
90* to each other and to the direction of propagation. The wave shown in the figure is the simplest
possible light wave. It is both monochromatic(has a siingle wavelength h) and linearly polarized(the eletric and
magnetic fields each oscillate along a single line)

In Physical optics, light is modeled as an electromagnetic transverse wave, a wave that oscillates
the electirc and magnetic fields perpendicular to the direction of propagation. The O

### Light Sources

The impact of lighting on our example shading model was quite simple it provided a dominant direction for shading.
of course, lighting in the real world can be quite complex. There can be multiple light sources each with ites own
size, shape, color, and intensity; indirect lighting adds even more variation. As we will see in the future, physically based,
photorealistic shading models need to take all these parameters into account.

In contrast, stylized shading models may use lighting in may different ways, depending on the needs of the
application and visual style. Some highly stylized models may have no concept of lighting at all,
or (like our Gooch shading example) may only use it to provide some simple directionality.

The next step in lighting complexity is for the shading model to react to the presence or absence of light
in a binary way. A surface shaded with such a model would have one appearance when lit nad a different appearance
when unaffected by light. This implies some criteria for distinguishing the two cases: distance from light sources,
shadowing(which will be discussed later), whether the surface is facing away from the light source(i.e.
the angle between the surface normal n and the lgiht vector l is greater than 90 degree), or some combination of these factors.

It's a small step from the binary presence or absence of light to a continous scale of light intensities.
This could be expressed as simple interpolation between absence and full presence, which implies a bounded
range for the intensity, perhaps 0 to 1, or as an unbounded quantity that affects the shading in some other way. A common
option for the latter is to factor the shading model into lit and unlit parts, with the light intensity k li
linearly scaling the lit part:


One of the most important tasks of a material system is dividing various shader functions into separate
elements and controlling how these are combined. There are many cases where this type
