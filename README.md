# Chapter 10 Bind cam

Previously, we fix the camera and had a good view on each stuff we added like the model and animations.

Now it's time to make the camera a first/third person one, with yaw and pitch.

It's mostly adapted from the examples and in the future, these mechanism will be public embers that you can use with some basic import.

We will create the camera entity, and bind it to the player as child().

Note that we also seperate the model and the player. This is for a better fine-tuning the position.

Pay attention to how child-parent and the local_to_parent() are used.
