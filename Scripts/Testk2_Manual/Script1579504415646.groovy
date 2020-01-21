import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.navigateToUrl('https://opensource-demo.orangehrmlive.com/')

WebUI.setText(findTestObject('Object Repository/Page_OrangeHRM/input_LOGIN Panel_txtUsername'), 'Admin')

WebUI.setText(findTestObject('Object Repository/Page_OrangeHRM/input_Username_txtPassword'), 'admin123')

WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/input_Password_Submit'))

WebUI.openBrowser('')

WebUI.navigateToUrl('https://opensource-demo.orangehrmlive.com/index.php/dashboard')

WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/img'))

WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/input__assignleavetxtEmployeeempName'))

WebUI.setText(findTestObject('Object Repository/Page_OrangeHRM/input__assignleavetxtEmployeeempName'), 'kiru')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_OrangeHRM/li_Employee Name   Invalid'), '1', true)

WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/input__assignleavetxtFromDate'))

WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/a_22'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_OrangeHRM/select_Full DayHalf DaySpecify Time'), 'half_day', 
    true)

WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/input__assignBtn'))

WebUI.click(findTestObject('Object Repository/Page_OrangeHRM/li_Employee Name   Invalid'))

WebUI.closeBrowser()

