package com.mycompany.app;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

import org.apache.poi.ss.usermodel.Cell;
import org.apache.poi.ss.usermodel.Row;
import org.apache.poi.ss.usermodel.Sheet;
import org.apache.poi.ss.usermodel.Workbook;
import org.apache.poi.xssf.usermodel.XSSFWorkbook;

import java.io.File;

/**
 * Hello world!
 *
 */
public class App 
{
    public static void main( String[] args )
    {
        FileInputStream file;
        try {
          file = new FileInputStream(new File("demo.xlsx"));
          System.out.println("File loaded");
          Workbook workbook = new XSSFWorkbook(file);

          Sheet sheet = workbook.getSheetAt(0);

          //Map<Integer, List<String>> data = new HashMap<>();
          //int i = 0;
          for (Row row : sheet) {
              //data.put(i, new ArrayList<String>());
              for (Cell cell : row) {
                  switch (cell.getCellType()) {
                      case STRING: 
                      case NUMERIC: 
                      case BOOLEAN: 
                      case FORMULA: 
                      default: System.out.println(cell.toString());
                  }
              }
              //i++;
          }
          workbook.close();
        } catch (FileNotFoundException e) {
          e.printStackTrace();
        } catch (IOException e) {
          e.printStackTrace();
        }

        System.out.println( "Hello World!" );
    }
}
