package com.mycompany.app;

/**
 * Hello world!
 *
 */
public class App 
{
    public static void main( String[] args )
    {
        String port = System.getenv("PORT");
        System.out.println("Port: " + port);
        
        System.out.println( "Hello World!" );
    }
}
