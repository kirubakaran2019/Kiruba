import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Keys

import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.openBrowser('')

WebUI.navigateToUrl('https://katalon-demo-cura.herokuapp.com/')

WebUI.click(findTestObject('Object Repository/Page_CURA Healthcare Service/a_Make Appointment'))

WebUI.click(findTestObject('Object Repository/Page_CURA Healthcare Service/div_Demo account_input-group'))

WebUI.setText(findTestObject('Object Repository/Page_CURA Healthcare Service/input_Username_username'), 'John Doe')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_CURA Healthcare Service/input_Password_password'), 'g3/DOGG74jC3Flrr3yH+3D/yKbOqqUNM')

WebUI.sendKeys(findTestObject('Object Repository/Page_CURA Healthcare Service/input_Password_password'), Keys.chord(Keys.ENTER))

WebUI.closeBrowser()

