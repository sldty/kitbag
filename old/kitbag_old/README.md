# kitbag

Base replication layer for Solidarity.

## Solidarity Overview

Let's get started.

Here's the deal.

Solidarity is going to be slightknack.dev 2.0.
fulfilling the ideal that the original website put in place.

## A Picture

It's saturday morning, 8:00am. You're writing a design outline for something.

The internet went out the night before, but you're not too worried - you see, you'd been working your design outline offline.

Whenever the internet comes back up, the outline will be encrypted, and uploaded to the wider solidarity network in a tit-for-tat manner. A couple hours pass by. Your internet comes back online, and you'd like your friend to look over it.

In the top left, you add your friend as an editor. In your shared chat, a message is automatically sent to notify your friend of the change. He sees it and hops on.

You've written your essay in a new markup language that your friend doesn't have installed. When he opens the document, the correct version of the language standard is pulled from the wider solidarity network, and is up and running in a flash.

You he tosses a chat onto the document:

"I think you have the right idea, but distributed really the way to go? A federated approach might work better"

You agree, both start reworking the document. While online, the document is synced live between you with the kitbag diff and resolution protocol. You're about done when the internet cuts out again. *sigh*. ISPs still haven't improved.

You keep working on your document anyway, all changes are saved to a local branch of the document. When your internet starts working again later you'll unify your and your friend's local branches.

You've polished what you can, but the internet's still not up. You open a video service.

The video service is an application, writing in a general purpose overarching language, that runs inside solidarity itself. Each service defines the layout and version permanent data-stores specific to that service. This is stored locally on the user's device, in a place the user can easily access from solidarity. because these data-stores are well-defined and built on the same technology as the rest of solidarity, it's quite easy to link services together and define translation layers between services. But you're not worrying about now.

Because you're offline, the video service can't pull videos from the video solidarity network. No worries. You've downloaded some videos and just watch those. One of them is about the design of federated systems - It's actually quite interesting.

You grab the video's page location, and toss it into the essay. Clicking on it will take you to the video, and you can see where it's from and what other pages link to it, which is nice for exploring other things.

The internet comes back up and you and your friend's local changes sync via kitbag diffing. Each change you makes records the version of the previous change, and the difference between that version and the new change.

Given that you own the document, The latest main version is your local deviation. You're notified that your friend's changes are present, but not merged. If your friend merges the changes on his end, then his branch will be automatically merged into yours given that no more conflicts exist. If you merge your friends changes into your branch, his will automatically update because not more conflicts exist. By default, all conflicts are marked, and resolution is as simple as accepting none, either, both, or nth. When branches match, they'll share the same hash. Trivial conflicts don't get in the way of getting work done - they're highlighted, but work can be done in other parts of the document.

Your design outline is looking good, so you change the view settings to public. This creates a new version of the page, marked as being public, that is accessible for everyone to view.

## We're going to implement the kitbag architecture outlined in this story.
