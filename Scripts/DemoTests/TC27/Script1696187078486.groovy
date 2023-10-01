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

WebUI.click(findTestObject('Object Repository/Page_NEVER STOP LEARNING - Automation Step by Step/div_CI CD DevOpsWhat is DevOpsJenkins Begin_a81f9e'))

WebUI.click(findTestObject('Object Repository/Page_NEVER STOP LEARNING - Automation Step by Step/a_What is DevOps'))

WebUI.switchToWindowTitle('DevOps - YouTube')

WebUI.click(findTestObject('Object Repository/Page_DevOps - YouTube/a_Difference between Continuous Integration_bad11f'))

WebUI.click(findTestObject('Object Repository/Page_Difference between Continuous Integrat_3b535f/button_An error occurred while retrieving s_72f8b5'))

WebUI.click(findTestObject('Object Repository/Page_Difference between Continuous Integrat_3b535f/div_Skip Ad'))

WebUI.click(findTestObject('Object Repository/Page_Difference between Continuous Integrat_3b535f/button_An error occurred while retrieving s_72f8b5_1'))

WebUI.switchToWindowTitle('NEVER STOP LEARNING - Automation Step by Step')

WebUI.click(findTestObject('Object Repository/Page_NEVER STOP LEARNING - Automation Step by Step/button_'))

