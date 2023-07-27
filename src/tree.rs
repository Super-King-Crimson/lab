//This will never work.
#[allow(unused)]
pub struct Node<'a, T> {
    data: T,
    parent: Option<&'a mut Node<'a, T>>,
    children: Vec<&'a mut Node<'a, T>>,
}
//First off, if a parent refers to its children, and children refer to their parents, then who owns the data?
//That means there has to be some ownership, but:
    //Multiple children can't own, let alone mutably own, one parent
    //If the parent owns its children, then you need ownership of a node to set its parent, which isn't correct

//Ok, let's pretend we have ownership of these values somewhere else in memory, and so that problem's solved.
//how would you change a node's parent
    //Step 1: remove them from their previous parent's children vector,
    //Step 2: set their current parent to their new parent's children vector,
    //Step 3: set their parent to an Some containing their new parent
//Do you see the problem?
//Let's do some lifetime annotations:
    //Step 1: &mut borrow of prev.children
    //Step 2: &mut borrow of self, &mut borrow of new.children
    //Step 3: &mut borrow of new.children (!!!!!!)
//Is that a double mutable borrow I see?

//This is unavoidable. Since the Some value of a parent must be a mutable reference to a Node
    //(otherwise we wouldn't be able to change the properties of a value through its children)
//there will always be two mutable references to new: one stored in a Some value in self, the other being the one passed into the function
//Look:

impl<'a, T> Node<'a, T> {
    //The reference to the other node must live as long as the current node
    pub fn set_parent(&'a mut self, other: &'a mut Node<'a, T>) {
        if let Some(prev) = &mut self.parent {
            //pretend this is some code that properly removes self from prev, just proving it's mutable
            prev.children.pop();
        }

        //mutable borrow 1
        other.children.push(self);
        //HERE IT IS! we refer to other to push self onto it, then we have to put that reference we pushed onto self, which is two mut refs 
        // self.parent = Some(other);

        //In this case we need a structure that allows multiple mutability: Rc<RefCell<T>>, otherwise this is flat-out impossible
    }
}