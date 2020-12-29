> ⚠ This repository is in a *very early* stage of development.
> Like ***very*** early. Not even guaranteed to *build* for goodness' sake.
> Documentation and advertised features may be incorrect.
> Pretend everything is written in future tense, if that helps.

# kitbag?
**kitbag!**

This is kitbag.¹⁻²

## What is kitbag, you ask?
Kitbag is a content-addressed versioned tree-structured graph-based datastore.

Kitbag is also a collection of cryptographically-secure distributed/federated gossip protocols for replicating and collaborating on data in real-time.

It's like the *backend* of Notion + Discord + Google Docs over BitTorrent.

It's a core component of the planned *Solidarity* project, a cornerstone of the Veritable Computation Initiative.

## What isn't kitbag?
Kitbag is not functional, at the moment (if you missed the disclaimer). I started it last Thursday, if that's any indication.

Kitbag, while technically being a library, is not designed to be general purpose. It may be extended in the future, but for now, it's just a simple MVP.

## What's the Veritable Computation Initiative?
Many issues seem daunting today because we do not yet have the tools to solve them. Veritable is an open initiative to develop new tools to solve today's technological problems. There are currently two publicized Veritable projects:

- Passerine, a small extensible programming language.
- Solidarity, a distributed platform built for communication and collaboration.

## How does kitbag work?
> Section not done.
> NOTE: some of this stuff is specific to Solidarity, specifically the hierarchy encoded in agents, namespaces, pages, etc. For this reason, the documentation here may not actually reflect the current state of kitbag, though it did at some point in the past.

Kitbag is the backend storage layer of Solidarity. Solidarity, as mentioned, is a distributed platform built for communication and collaboration.

What sorts of communication and collaboration?

Like the world wide web, Solidarity is composed of pages, which can be shared with others. Pages can contain other pages and so on. Users can share pages, which can be collaborated on in real-time - here's what that might look like:

![](https://raw.githubusercontent.com/slightknack/kitbag/master/mock.png)

Note that this is just one type of page - a document, and that pages can also be chats, videos, spreadsheets, presentations, and so on.

### Content
The first concept to understand in the concept of an *Agent*. An agent is a person, a group of people, a bot, or so on, whose identity is verified through public-key cryptography.

Each agent owns any number of *Namespaces*. A namespace can be thought of as a workspace that can be shared with other agents - an indirect analog might be a website, a discord server, a git repository, a slack workspace - you get the point.

Each namespace is composed of any number of *Pages*. Each namespace has a base page, i.e. the root page, from which all other pages originate. Pages are organized in a nested tree-like fashion. Pages also be embedded directly into other pages. They also record links and backlinks. In this sense there is a base tree hierarchy of Pages, but because each page can link to others arbitrarily and backlinks are recorded, a graph of pages is created.

### Data
What can a page be, though? Each page contains a singular *Data*. This data may be something like a markdown document, or a text file. It can also be something more complex, like an image, a video, a chatroom, and so on. Further down the road, we plan to use some web-assembly and multi-format magic to allow pages to be arbitrary applications.

### Datastore
All this information is kept in a *Datastore*. A datastore, or as we like to call it, a *kitbag*, keeps track of two things:

1. A content-addressed store of *Data*.
2. The histories of ever-changing *Content* (like Agents, Namespaces, and Pages).

Now the datastore is not fully content-addressed - instead of storing all the content directly, diffs in the history of *Content* are preserved. To look up any piece of content, diffs are used to construct the content (which is then verified against the address) if it is not cached on-hand.

However, kitbag is a distributed datastore, so you might not have everything on-hand locally...

### Nodes
Each running instance of a datastore has a *Node* through which it communicates with the wider solidarity network to collaboratively get and work on data. For example, when a friend sends a document to you, they send the document's base history. When you try to first access the document, seeing that you do not have it on-hand, your node queries your friend's node for the appropriately addressed content. Nodes will implement a bit-swap protocol similar to the one used by IPFS.

Unlike IPFS, however, nodes will always only send the diffs that the other node needs. How does the node know which diffs to send?

### Branches
To keep the datastore fast and local-first, and to avoid expensive locking and merging with every change, *Branches* are used. A branch is similar to a git branch, in the sense that it's a separate space where one is free to work on a document. Unlike git branches, however, kitbag branches are entirely local first - there is no shared upstream master, rather, master for you *is* your local master branch.

Unlike git, however, you keep track of your peers' branches as well. These changes are synced in real-time, and because branches are separate, you never need to worry about two people 'pushing' to the same branch at the same time. (I use air quotes here because this process, of branching and committing, is entirely automatic.)

How do we ensure that:

- Branches don't diverge,
- Conflicts can be resolved,
- And changes are authorized?

When any content is queried, from the datastore, the datastore tries to automatically and efficiently merge all non-local branches into your branch. If no conflicts arise, this merge is recorded and the change is propagated to peers, which then may automatically merge given no further conflicts.

If conflicts do arise, then they are returned when the document is queried. These conflicts will be presented to agents along with the rest of the document, who can then can easily resolve such conflicts by selecting 'use mine' or 'use theirs' or 'use both' and so on. Note this this merging is done in an automatic and incremental manner, so agents collaborating on a document can see merge conflicts arise in real-time as they occur. Humans are usually pretty good at collaborating over a fast network connection - where merging really shines is when you're offline for a long period of time.

If this is the case, once you are back online, collaborating branches will be downloaded and non-conflicting merges will be applied. The unresolved merges will be highlighted, which can then easily be resolved as discussed above.

Can anyone resolve changes? Who can see or make changes to some content in the first place?

### Permissions
Of course, each item of content has associated permissions. Each item has a base permission, i.e. what everyone in the world can do with it, a set of permissions for specific agents, as well as a definitive owner.

> TODO.

### Versioning and Diffing
All Data is content addressed, meaning that it is identified by its hash, rather than a name. This means that all versions of any item of data are recorded.

This might seem a bit expensive, though. A one-letter change to a document changes its hash, so does that imply that we have to store the entire document with every change?

Luckily enough, no. Instead of storing the new document along with the old document, we simply calculate the difference between the two documents. These differences are very small, and carry little associated metadata, making a one-letter-change, well, a one-letter change.

This is done by recording the history of the document:

> History of Some Document:
>
> 0 - Data Address → Delta Base  
> 1 - Data Address → Delta Tip  
> 2 - Data Address → Delta Tip  
> n - ...  

Each item in history points to the item before it.

What if we have a massive history?

> History of Some Document:
>
> 998 - ...  
> 999 - Data Address → Delta Tip  

Does this mean *all* the diffs have to be applied to construct the document? No!

Because the datastore is content addressed, we first check to see if we have a cached version of the document on hand before going further back in history. These 'checkpoints,' so to speak, reduce the amount of work to retrieve any version of a document to constant, as opposed to linear, time.

If data is also addressed by its hash, how do we compare successive versions of the same document?

Each branch (as discussed earlier) maps content to their histories. This makes it easy to retrieve the history of any document. We can easily compare the differences between documents by using the diffing tools made readily available.

> How does the node know which diffs to send?

Because the datastore keeps track of different branches, and nodes keep a record of the shared data peer nodes have, it is quite simple to calculate the difference between what you have and what the peer has. This difference is what is sent over the wire - because the peer node has the content you based the diff on, the peer does not need to request any more data from your node, and can simply directly apply the change to your branch.

## Usage
Here's a brief example of what working with kitbag is like:

```rust
// The page is not a reference, the program is free to modify it however it likes.
let mut page = datastore.get(page_location);
page.data = Data::Document("Deleted everything".to_string());

// Changes are recorded by updating the datastore.
// This operation is inexpensive for small changes
// and should can be called whenever necessary.
datastore.update(page);
```

## Well?

If you've read this far, thanks! You've probably formed some opinions on the topic - If you have any ideas, reach out 😉.

---

### Footnotes

1. > **kitbag**  
   > n. A bag to hold a sailor's kit.  
   > n. A knapsack.  
   > n. A duffel bag.

2. kitbag is HRDB v2.0. What is an HRDB? A Human Readable Data-Base is a content-addressed versioned data-store (and, despite it's name, not that human-readable at all).
