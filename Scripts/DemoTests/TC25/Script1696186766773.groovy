import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://automationstepbystep.com/')

WebUI.click(findTestObject('Object Repository/Page_NEVER STOP LEARNING - Automation Step by Step/a_Udemy'))

WebUI.click(findTestObject('Object Repository/Page_Udemy Courses - Automation Step by Step/button_'))

WebUI.click(findTestObject('Object Repository/Page_Udemy Courses - Automation Step by Step/font_Referral Price'))

WebUI.switchToWindowTitle('API Test Automation | POSTMAN | SOAPUI | KATALON | JMETER | Udemy')

WebUI.click(findTestObject('Object Repository/Page_API Test Automation  POSTMAN  SOAPUI  _bae340/span_Log in'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_Just a moment/h1_www.udemy.com'), 'www.udemy.com')

