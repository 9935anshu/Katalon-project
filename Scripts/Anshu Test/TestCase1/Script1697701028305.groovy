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

WebUI.closeBrowser()

WebUI.openBrowser('')

WebUI.navigateToUrl('https://ecommerce-playground.lambdatest.io/index.php?route=account/login')

WebUI.click(findTestObject('Object Repository/Page_Account Login/a_Register'))

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_First Name_firstname'), 'anshu')

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_Last Name_lastname'), 'singh')

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_E-Mail_email'), 'anshu1@binmile.com')

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_Telephone_telephone'), '9879797872')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Register Account/input_Password_password'), '4nvbrPglk7k=')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Register Account/input_Password Confirm_confirm'), 'iGDxf8hSRT4=')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Register Account/input_Password_password'), 'iGDxf8hSRT4=')

WebUI.click(findTestObject('Object Repository/Page_Register Account/label_I have read and agree to the Privacy Policy'))

WebUI.click(findTestObject('Object Repository/Page_Register Account/input_Privacy Policy_btn btn-primary'))

