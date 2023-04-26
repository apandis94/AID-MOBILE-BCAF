<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5. PPKD - Asuransi Dicadangkan</name>
   <tag></tag>
   <elementGuidId>b10e3615-ac2e-469e-968a-8eb17d415b6c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${accno}\&quot;,\n  \&quot;JenisCadangan\&quot;: \&quot;${jenis_cad}\&quot;,\n  \&quot;Persentase\&quot;: ${percent_ins_cad},\n  \&quot;NominalKomisi\&quot;: ${komisi_amnt}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0c65da22-3fde-4504-891f-811f7e3a6e61</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>f4d23c91-0326-49e0-9f52-3737eca0ea4f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/KomisiAsuransi/Dicadangkan/NominalKomisi</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Cadangan Dana Program'</defaultValue>
      <description></description>
      <id>0e25cb4d-fc71-440f-8ff5-c67cab8ac640</id>
      <masked>false</masked>
      <name>jenis_cad</name>
   </variables>
   <variables>
      <defaultValue>'9530708518-001'</defaultValue>
      <description></description>
      <id>89850ded-d2f7-4efd-bff3-ba1d8fd38b2e</id>
      <masked>false</masked>
      <name>accno</name>
   </variables>
   <variables>
      <defaultValue>5.0</defaultValue>
      <description></description>
      <id>143c1f53-4877-43d2-9138-2b0cf0ec4bd3</id>
      <masked>false</masked>
      <name>percent_ins_cad</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>f56f5ee9-1ebb-4d9b-8fcb-935fa7cc7582</id>
      <masked>false</masked>
      <name>komisi_amnt</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
