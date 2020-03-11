
package di_keywords

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.testdata.DBData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper
import internal.GlobalVariable

import java.sql.Connection
import java.sql.DriverManager
import java.sql.ResultSet
import java.sql.Statement
import org.openqa.selenium.TakesScreenshot
import org.openqa.selenium.WebDriver
import org.openqa.selenium.chrome.ChromeDriver
import org.openqa.selenium.chrome.ChromeOptions
import org.openqa.selenium.OutputType
import org.apache.commons.io.FileUtils

public class DI_API_DB_Keywords {

	public static JsonSlurper jsonSlurper = new JsonSlurper()
	private static Connection connection = null;

	@Keyword
	def int createNewUser(String grant_type, String username, String password, String scope, String token, int expectedStatus) {
		def response = WS.sendRequestAndVerify(findTestObject("Get Swagger Access token",
				["grant_type": grant_type, "username": username, "password": password, "scope": scope,"refresh_token": token ]))

		System.out.print("aaa"+response.statusCode+"kkk"+response.getResponseBodyContent()+"\n");
		System.out.println("aaa test");


		//def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		return response.statusCode
	}

	@Keyword
	def void verifyHelathAPIResponse(String Response) {
		System.out.print("aaa"+response.statusCode+"kkk"+response.getResponseBodyContent()+"\n");
		System.out.println("aaa test");


		//def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		//return response.statusCode
	}

	@Keyword
	def connectToDataBase(String dbname) {
		//Load driver class for your specific database type
		String port = GlobalVariable.port
		String url = GlobalVariable.host
		String conn = "jdbc:postgresql://"+ url + ":" + port + "/" + dbname;

		if(connection != null && !connection.isClosed())
		{

			connection.close()

		}

		connection = DriverManager.getConnection(conn, "rbmadmin", "P2ca9GK2OGUQAHe");

	}


	@Keyword
	def static void findUserById(int id, int age, String username, String password, String gender, int expectedStatus) {
		WS.sendRequestAndVerify(findTestObject('Object Repository/GET user by id', [('id') : id]))
	}

	@Keyword
	def disconnectDatabase()
	{
		if(connection != null && !connection.isClosed())
		{
			connection.close()

		}
		connection = null
		System.out.println("Closed Connection");
	}

	@Keyword
	def test()
	{
		println ("test")
		ChromeOptions c = new ChromeOptions();
		WebDriver driver = new ChromeDriver(c);
		System.out.print("aaa")
		driver.get("https://dev.trialoversight.io/");
		File src= ((TakesScreenshot)driver).getScreenshotAs(OutputType.FILE);
		try {
			// now copy the  screenshot to desired location using copyFile //method
			FileUtils.copyFile(src, new File("C:\\Users\\alekhya.vallabhaneni\\Desktop\\sample.png"));
			println ("done")
		}

		catch (IOException e)
		{
			System.out.println(e.getMessage());

		}
	}

	@Keyword
	def queryExecution(String query)
	{

		Statement stm = connection.createStatement();
		ResultSet rs = stm.executeQuery(query);

		if (rs.next() == false) { System.out.println("ResultSet in empty in Java"); }
		else {
			while (rs.next()) { String data = rs.getString("src_study_id"); System.out.println(data);

				String data1 = rs.getString("study_name"); System.out.println(data1);
			}
		}

	}

	@Keyword
	def fetchRecords()
	{
		DBData d = TestDataFactory.findTestData("Data Files/Study Query")
		String id= d.getValue(1,1)
		List a = d.getAllData()

		System.out.println("length"+a.size()+"1st SrcID"+id);

	}
	@Keyword
	def excelFile(String data)
	{
		println("ddd"+data)


	}

}


