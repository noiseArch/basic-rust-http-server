# Basic HTTP Server made with Rust

[ES]
Hice este proyecto siguiendo el tutorial en [[https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html]]
Se trata de entender como funcionan frameworks back-end, como gestionan el routing y otras funcionalidades como por ejemplo la página 404
Es un servidor multiproceso por lo que es capaz de gestionar multiples requests a la vez. Para recibir las requests, el protocolo TCP escucha las llamadas a un puerto especificado por el programado. La parte mas dificil fue hacerlo multiproceso ya que tenia que implementar canales, tareas, ThreadPool para que los workers que estaban sin tareas trabajen en las tareas que iban llegando a la queue, para al completarlas volver al pool y estar listos para otra tarea. 

[EN]
I made this proyect following this tutorial [[https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html]]
It's about understanding how back-end frameworks work, how they manage routing and other functionalities such as the 404 page.
It's a multi-threaded server so it's capable of managing multiple requests at the same time. To receive requests, the TCP protocol listens for calls to a port specified by the programmer. The most difficult part was making it multi-threaded since I had to implement channels, tasks, ThreadPool so that the workers who were without tasks could work on the tasks that were arriving in the queue, so that when they were completed they would return to the pool and be ready to another task.