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

WebUI.navigateToUrl('https://ecommerce-playground.lambdatest.io/index.php?route=account/login')

WebUI.setText(findTestObject('Object Repository/Page_Account Login/input_E-Mail Address_email'), 'anshu@binmile.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Account Login/input_Password_password'), 'iGDxf8hSRT4=')

WebUI.click(findTestObject('Object Repository/Page_Account Login/input_Forgotten Password_btn btn-primary'))

WebUI.click(findTestObject('Object Repository/Page_My Account/a_Modify your address book entries'))

WebUI.click(findTestObject('Object Repository/Page_Address Book/a_New Address'))

WebUI.setText(findTestObject('Object Repository/Page_Address Book/input_First Name_firstname'), 'ansu')

WebUI.setText(findTestObject('Object Repository/Page_Address Book/input_Last Name_lastname'), 'ss')

WebUI.setText(findTestObject('Object Repository/Page_Address Book/input_Company_company'), '')

WebUI.click(findTestObject('Object Repository/Page_Address Book/input_Company_company'))

WebUI.setText(findTestObject('Object Repository/Page_Address Book/input_Address 1_address_1'), 'fe')

WebUI.setText(findTestObject('Object Repository/Page_Address Book/input_City_city'), 'xcd')

WebUI.setText(findTestObject('Object Repository/Page_Address Book/input_Post Code_postcode'), '222')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Address Book/select_--- Please Select ---  Aaland Island_56731b'), 
    '222', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Address Book/select_--- Please Select ---  Aaland Island_56731b'), 
    '220', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Address Book/select_--- Please Select --- Cherkaska Obla_7d7235'), 
    '3495', true)

WebUI.click(findTestObject('Object Repository/Page_Address Book/input_Back_btn btn-primary'))

WebUI.setText(findTestObject('Object Repository/Page_Address Book/input_Address 1_address_1'), 'fe234325')

WebUI.click(findTestObject('Object Repository/Page_Address Book/input_Back_btn btn-primary'))

WebUI.closeBrowser()

