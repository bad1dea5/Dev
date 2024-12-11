---
layout: default
title: Rust Traits
date: 2024-11-12 22:07:03 +0000
categories: rust
---
## {{ page.title }}
{: .d-flex .justify-content-between .flex-wrap .flex-md-nowrap .align-items-center .pt-3 .pb-2 .mb-3 .border-bottom }

###### {{ page.date }}
{: .d-flex .justify-content-between .flex-wrap .flex-md-nowrap .align-items-center .pb-3 .mb-4}

A `trait` is a collection of methods defined for an unknown type: `Self`.
They can access other methods declared in the same `trait`, and can be
implemented for any data type.

{% highlight rust %}
trait Foo<T> {
    fn new() -> Self;
    fn test(&self, s: &String);
}
{% endhighlight %}

{% highlight rust %}
struct Bar<T> {
    value: T
}

impl<T> Foo<T> for Bar<T>
where
    T: Sized
{
    fn new() -> Self {
        Bar {
            value: 1
        }
    }

    fn test(&self, s: &String) {
        println!(s);
    }
}
{% endhighlight %}

{% highlight rust %}
let bar: Bar = Bar::<i32>::new();
bar.test(format!("hello"));
{% endhighlight %}
